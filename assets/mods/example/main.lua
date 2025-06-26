function Main()
    print(simple_warfare.game_info())
    print(mod_info)
    mod_enable:enable_js("tank.js",{"Tank"})
    mod_enable:enable_js("builder.js",{"Builder"})
end
