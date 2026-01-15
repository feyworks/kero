---@meta

---@class (exact) Font: FontMethods

---@class FontClass: FontMethods
local module = {}

---@class FontMethods
local methods = {}

---Create a new empty font.
---@param size number
---@param pixelated boolean
---@return Font
---@nodiscard
function module.new(size, pixelated) end

---Load and rasterize a font from a TTF file.
---@param path string
---@param size number
---@param pixelated boolean
---@param chars string?
---@return Font
---@nodiscard
function module.from_ttf_file(path, size, pixelated, chars) end

---The font's baked size.
---@param self Font
---@return number
---@nodiscard
function methods.size(self) end

---If the font is pixelated.
---@param self Font
---@return number
---@nodiscard
function methods.pixelated(self) end

---Set the character's render glyph.
---@param self Font
---@param chr string
---@param subtexture SubTexture?
---@param advance number
function methods.set_glyph(self, chr, subtexture, advance) end

-- ---Get the glyph subtexture and advance for the character.
-- ---@param self Font
-- ---@param chr string
-- ---@return SubTexture?
-- ---@return number
-- ---@nodiscard
-- function methods.glyph(self, chr) end

---Set the kerning when rendering from one character to another.
---@param self Font
---@param left string
---@param right string
---@param kerning number
function methods.set_kerning(self, left, right, kerning) end

---Get the kerning when rendering from one character to another.
---@param self FontMethods
---@param left string
---@param right string
---@return number
---@nodiscard
function methods.kerning(self, left, right) end

return module