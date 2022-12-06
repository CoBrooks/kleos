use alloc::{collections::BTreeMap, string::{String, ToString}};
use core::fmt::Write;

use tracing::{Subscriber, Metadata, span, field::Visit, Level};

use crate::{serial_println, Locked, println, color::{ColorExt, ColorName}};

pub fn init_tracing() {
    tracing::subscriber::set_global_default(Locked::new(KernelTracingSubscriber::new()))
        .unwrap();
}

struct SpanVisitor<'a> {
    record: &'a mut String
}

impl Visit for SpanVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn core::fmt::Debug) {
        writeln!(self.record, "  {}: {:?}", field.name(), value).unwrap()
    }
}

struct EventVisitor<'a> {
    record: &'a mut String,
}

impl Visit for EventVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn core::fmt::Debug) {
        if field.name() == "message" {
            write!(self.record, "{value:?}").unwrap()
        } else {
            writeln!(self.record, "{}: {:?}, ", field.name(), value).unwrap()
        }
    }
}

struct KernelSpan {
    level: Level,
    name: String,
    file: Option<String>,
    line: Option<u32>,
    record: String
}

impl KernelSpan {
    fn new(meta: &Metadata, record: String) -> Self {
        KernelSpan {
            level: *meta.level(),
            name: meta.name().to_string(),
            file: meta.file().map(|s| s.to_string()),
            line: meta.line(),
            record
        }
    }
}

impl core::fmt::Display for KernelSpan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let KernelSpan { level, name, file, line, record } = self;

        let level_text = alloc::format!("{:5}", level);
        let level_color = match *level {
            Level::DEBUG => ColorName::Blue,
            Level::INFO  => ColorName::Green,
            Level::WARN  => ColorName::Yellow,
            Level::ERROR => ColorName::Red,
            Level::TRACE => ColorName::Black,
        };
        let level = level_text.fg(level_color);

        if let Some(file) = file && let Some(line) = line.map(|c| c.to_string()) {
            let file = file.fg(ColorName::White);
            let line = line.fg(level_color);

            write!(f, "[{level} {file}:{line}]\n{name}\n{record}\n  ")
        } else {
            write!(f, "[{level}]\n{name}\n{record}\n  ")
        }
    }
}

struct KernelTracingSubscriber {
    spans: BTreeMap<u64, KernelSpan>,
    current_span: Option<u64>,
    next_id: u64,
}

impl KernelTracingSubscriber {
    const fn new() -> Self {
        KernelTracingSubscriber {
            spans: BTreeMap::new(),
            current_span: None,
            next_id: 1,
        }
    }

    fn new_span(&mut self, span: &span::Attributes) -> span::Id {
        let mut record = String::new();
        let mut visitor = SpanVisitor { record: &mut record };
        span.record(&mut visitor);

        let kspan = KernelSpan::new(span.metadata(), record);

        self.spans.insert(self.next_id, kspan);
        
        let id = span::Id::from_u64(self.next_id);
        self.next_id += 1;
        id
    }

    fn enter(&mut self, span: &span::Id) {
        self.current_span.replace(span.into_u64());
    }
    
    fn exit(&mut self, _: &span::Id) {
        self.current_span.take();
    }
}

impl Subscriber for Locked<KernelTracingSubscriber> {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true // Interested in every trace emitted
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        let mut sub = self.lock();
        sub.new_span(span)
    }

    fn record(&self, span: &span::Id, values: &span::Record<'_>) {
        // TODO: record span information
        serial_println!("span {span:?}: {values:#?}");
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        // TODO: keep track of span order
        serial_println!("{span:?} -> {follows:?}");
    }

    fn event(&self, event: &tracing::Event<'_>) {
        let mut event_info = String::new();
        let mut visitor = EventVisitor { record: &mut event_info };
        event.record(&mut visitor);
        
        let sub = self.lock();

        if let Some(span_id) = sub.current_span && let Some(span) = sub.spans.get(&span_id) {
            println!("{span}{event_info}");
        } else {
            // TODO: abstract
            let meta = event.metadata();

            let level_text = alloc::format!("{:5}", meta.level());
            let level_color = match *meta.level() {
                Level::DEBUG => ColorName::Blue,
                Level::INFO  => ColorName::Green,
                Level::WARN  => ColorName::Yellow,
                Level::ERROR => ColorName::Red,
                Level::TRACE => ColorName::Black,
            };
            let level = level_text.fg(level_color);

            let file = meta.file().unwrap_or("UNKNOWN").fg(ColorName::White);
            let line = meta.line().unwrap_or(0).to_string();
            let line = line.fg(level_color);

            println!("[{level} {file}:{line}] {event_info}");
        }
    }

    fn enter(&self, span: &span::Id) {
        let mut sub = self.lock();
        sub.enter(span)
    }

    fn exit(&self, span: &span::Id) {
        let mut sub = self.lock();
        sub.exit(span)
    }
}
