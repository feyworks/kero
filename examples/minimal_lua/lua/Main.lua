local Keyboard = require "Keyboard"
local Key      = require "Key"
local Mouse    = require "Mouse"
local Vec2     = require "Vec2"
local Color    = require "Color"
local Draw     = require "Draw"
local Line     = require "Line"

local Main     = {}

function Main:init()

end

function Main:update()
    if Keyboard.pressed(Key.SPACE) then
        print("SPACE!")
    end
end

function Main:render()
    local m = Mouse.pos()
    local line = Line.new(Vec2.zero(), m)
    Draw.line(line, Color.red())
end

return Main
