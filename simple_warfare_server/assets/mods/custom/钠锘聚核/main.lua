function Main()
    mod_manager:add_js("单位/向日葵/向日葵.js", {"Sunflower"})
    Navigation()
end

function Navigation()
    navigator_layer_manager:add_layers("导航层/land.toml")
end
