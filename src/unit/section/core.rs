use boa_engine::{JsArgs, JsResult, class::*, prelude::*, value::TryFromJs};

#[derive(Debug, Trace, Finalize, JsData, TryFromJs)]
pub struct Core {
    pub name: JsString,
    pub hp: u32,
    pub price: u32,
}

impl Class for Core {
    const NAME: &'static str = "Core";

    const LENGTH: usize = 3;
    fn data_constructor(
        _new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<Self> {
        let name = args.get_or_undefined(0).to_string(context)?;
        let hp = args.get_or_undefined(1).to_u32(context)?;
        let price = args.get_or_undefined(1).to_u32(context)?;
        let core = Self { name, hp, price };
        Ok(core)
    }

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        Ok(())
    }
}
