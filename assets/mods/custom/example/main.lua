function Main()
    mod_manager:add_js("tank/tank.js", {"Tank"})
    map_manager:add_map("maps/[p2]Small_Island (2p).tmx")
    Navigation()
end

function Navigation()
    navigator_layer_manager:add_layers("导航层/land.toml")
end

