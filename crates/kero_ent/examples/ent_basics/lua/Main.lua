local Keyboard = require "Keyboard"
local Key      = require "Key"
local App      = require "App"
local World    = require "World"
local Entity   = require "Entity"
local Counter  = require "Counter"
local Screen   = require "Screen"
local Color    = require "Color"
local Mover    = require "Mover"

local Main     = {}

function Main:init()
    self.screen = Screen.new_frame(320, 180, false)

    self.world = World.new()

    local ent = Entity.new_at(60, 40)
    ent:add(Mover.new(0xffffff))
    self.counter = ent:add(Counter.new(0xffffff))
    self.world:add(ent)
end

function Main:update()
    self.screen:update()

    self.world:update()

    if Keyboard.pressed(Key.SPACE) then
        self.counter:reset()
    end

    -- restart the game, reloading all the Lua code
    if Keyboard.pressed(Key.R) then
        App.restart()
    end
end

function Main:render()
    self.screen:set_as_draw_surface(Color.black())

    self.world:render()
    
    self.screen:draw_to_window(Color.black())
end

return Main
