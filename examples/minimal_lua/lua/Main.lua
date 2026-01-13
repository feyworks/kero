local Keyboard = require "Keyboard"
local Key      = require "Key"
local App      = require "App"

local Main     = {}

function Main:init()
    print("init")

    print(App.cache_dir())
    print(App.config_dir())
    print(App.config_local_dir())
    print(App.data_dir())
    print(App.data_local_dir())
    print(App.preferences_dir())
end

function Main:update()
    if Keyboard.pressed(Key.ESCAPE) then
        App.quit()
    end
    if Keyboard.pressed(Key.R) then
        App.restart()
    end
end

function Main:render()
    
end

return Main
