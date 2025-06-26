use boa_engine::{JsArgs, JsResult, class::*, prelude::*, value::TryFromJs};

#[derive(Debug, Trace, Finalize, JsData, TryFromJs)]
pub struct Graphics {
    image: JsString,
}

impl Class for Graphics {
    const NAME: &'static str = "Graphics";

    fn init(_class: &mut ClassBuilder<'_>) -> JsResult<()> {
        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<Self> {
        let image = args.get_or_undefined(0).to_string(context)?;
        Ok(Graphics { image })
    }
}
