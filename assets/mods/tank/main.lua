function Main()
    print(simple_warfare.game_info())
    mod_info:load_class("tank.js")
    mod_info:enable_unit("Tank")
end
