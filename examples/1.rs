/*
                                let code_js = r#"
            import { CustomUnit, Core } from "./simple_warfare_engine.js";

            class Tank extends CustomUnit {
                constructor() {
                    let core = new Core("坦克", 1000, 1000);
                    super(core);
                };
            };

            export{Tank};
            "#;

                let module = Module::parse(
                    Source::from_reader(code_js.as_bytes(), Some(Path::new("./tank.mjs"))),
                    Some(module.realm().clone()),
                    context,
                )
                .unwrap();
                let promise = module.load_link_evaluate(context);
                tx.send(SmilodonEngineEvent::EngineInited)?;
                context.run_jobs();

                assert_eq!(
                    promise.state(),
                    PromiseState::Fulfilled(JsValue::undefined())
                );

                let binding = module
                    .namespace(context)
                    .get(js_string!("Tank"), context)
                    .unwrap();
                let tank_obj = binding.as_object().ok_or("not found obj").unwrap();

                let tank = tank_obj.construct(&[], None, context).unwrap();
                info!("{:?}", tank.get(js_string!("name"), context)?);

                tank.set(js_string!("name"), js_string!("超级坦克"), true, context)?;
                info!("{:?}", tank.get(js_string!("name"), context)?);

                 */