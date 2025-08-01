function Main()
    mod_manager:add_js("tank/tank.js", {"Tank"})
    mod_manager:add_js("builder.js", {"Builder"})
    map_manager:add_map("maps/[p2]Small_Island (2p).tmx")
    grid_manager:create_grid("Land", {"water", "lava"})
end
