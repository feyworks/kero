---@meta

---@alias ColorMode integer

---@class ColorModeModule
---@field MULT ColorMode
---@field WASH ColorMode
---@field VETO ColorMode
---@field MISC ColorMode
local ColorMode = {}

---Create a new color mode.
---@param mult number `0-1`
---@param wash number `0-1`
---@param veto number `0-1`
---@param misc number `0-1`
function ColorMode.new(mult, wash, veto, misc) end

---Blend between two color modes.
---@param from ColorMode Starting color mode.
---@param to ColorMode Ending color mode.
---@param factor number Blend factor from `0-1`.
function ColorMode.blend(from, to, factor) end

---Get the mode's mult value.
---@param mode ColorMode
---@return number
---@nodiscard
function ColorMode.mult(mode) end

---Get the mode's wash value.
---@param mode ColorMode
---@return number
---@nodiscard
function ColorMode.wash(mode) end

---Get the mode's veto value.
---@param mode ColorMode
---@return number
---@nodiscard
function ColorMode.veto(mode) end

---Get the mode's misc value.
---@param mode ColorMode
---@return number
---@nodiscard
function ColorMode.misc(mode) end

return ColorMode
