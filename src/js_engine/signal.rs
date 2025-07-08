use boa_engine::prelude::*;

pub type JsSignal = JsObject;
/*
#[derive(Default, Trace, Finalize, JsData)]
pub struct HostDefinedSignalSystem {
    #[unsafe_ignore_trace]
    pub signal_emit_queue: Vec<EmitSignal>,
}

impl HostDefinedSignalSystem {
    pub fn insert_emit_signal(&mut self, emit_signal: EmitSignal) {
        self.signal_emit_queue.push(emit_signal);
    }
}

#[derive(Default, Trace, Finalize)]
pub struct EmitSignal {
    pub signal: JsSignal,
    pub args: Vec<JsValue>,
}

impl EmitSignal {
    pub fn new(signal: JsSignal, args: impl Into<Vec<JsValue>>) -> Self {
        Self {
            signal,
            args: args.into(),
        }
    }
}

#[derive(Debug, Clone, JsData, Trace, Finalize, TryIntoJs, TryFromJs)]
pub struct Signal {
    pub name: JsString,
    pub connect: Vec<JsFunction>,
}

impl Signal {
    pub fn new(name: JsString) -> Self {
        Self {
            name,
            connect: Vec::new(),
        }
    }
}

pub fn init_signal() -> Result<Runtime> {
    let rt = Runtime::new()?;

    Ok(rt)
}

impl Class for Signal {
    const NAME: &'static str = "Signal";
    const LENGTH: usize = 1;
    fn data_constructor(
        _new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<Self> {
        let name = args.get_or_undefined(0).to_string(context)?;
        let this = Self::new(name);

        let sw = context
            .global_object()
            .get(js_string!("sw"), context)?
            .to_object(context)?;

        sw.call(
            &JsValue::Object(Self::try_into_js(&this, context)?.to_object(context)?),
            &[],
            context,
        )?;

        Ok(this)
    }

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        class.method(js_string!("connect"), 1, unsafe {
            NativeFunction::from_closure(|referrer, args, ctx| Ok(JsValue::Undefined))
        });
        Ok(())
    }
}
*/
