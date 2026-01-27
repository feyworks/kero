local SpritePacker = require "SpritePacker"
local Vec2         = require "Vec2"
local Screen       = require "Screen"
local Time         = require "Time"
local Num          = require "Num"
local Rect         = require "Rect"

local Main     = {}

function Main:init()
    self.screen = Screen.new_frame(320, 180, false)

    local packer = SpritePacker.new()
    packer:add_ase("player", "../basics/assets/player.aseprite")
    packer:add_sprite("portrait", "../basics/assets/portrait.png", true, 0)
    packer:add_sheet("tiles", "../basics/assets/tiles.png", true, 16, 16, 0)
    packer:add_patch("textbox", "../basics/assets/textbox.png", true, 8, 8, 16, 16)
    packer:add_font("virtue", "../basics/assets/virtue.ttf", 16)

    self.atlas = packer:pack(4096)

    self.frame_idx = 0
    self.frame_timer = 0
    self.frame = 0
    self.tags = self.atlas.anims.player:tags()
end

function Main:update()
    self.screen:update()

    local dir = Num.floor(Time.since_startup() / 2.0) % 4
    local tag = self.tags[1 + dir]

    local frame_duration = self.atlas.anims.player:frame_duration(self.frame)

    self.frame = tag.from + self.frame_idx
    self.frame_timer = self.frame_timer + Time.delta()
    if self.frame_timer >= frame_duration then
        self.frame_timer = self.frame_timer - frame_duration
        self.frame_idx = (self.frame_idx + 1) % 4
    end
end

function Main:render()
    self.screen:set_as_draw_surface(0x286d38ff)

    self.atlas.sprites.portrait:draw(Vec2(0, 111))

    local tiles = self.atlas.sheets.tiles
    for row = 0, tiles:rows() - 1 do
        for col = 0, tiles:cols() - 1 do
            local x = col * tiles:tile_w() + 48
            local y = row * tiles:tile_h() + 16
            tiles:draw_tile(col, row, Vec2(x, y))
        end
    end

    self.atlas.anims.player:draw(self.frame, Vec2(216, 40))

    local rect = Rect(80, 120, 232, 56)
    local wave = Num.round(Time.wave(-3, 3, 2))
    self.atlas.patches.textbox:draw(rect:inflate(wave, -wave))

    self.atlas.fonts.virtue:draw_text("Patches can be used to draw things like", Vec2(96, 145))
    self.atlas.fonts.virtue:draw_text("textboxes and menu containers!", Vec2(96, 157))

    self.screen:draw_to_window(0x000000ff)
end

return Main
