---@meta

---@class (exact) SpriteFont: SpriteFontMethods

---@class SpriteFontClass: SpriteFontMethods
module = {}

---@class SpriteFontMethods
local methods = {}

---The font's ascent (offset of top to baseline).
---@param self SpriteFont
---@return number
---@nodiscard
function methods.ascent(self) end

---The font's descent (offset of bottom to baseline).
---@param self SpriteFont
---@return number
---@nodiscard
function methods.descent(self) end

---The font's height (`ascent - descent`).
---@param self SpriteFont
---@return number
---@nodiscard
function methods.height(self) end

---The font's desired gap between lines of text.
---@param self SpriteFont
---@return number
---@nodiscard
function methods.line_gap(self) end

---The kerning for the character pair.
---@param self SpriteFont
---@param left string
---@param right string
---@return number
---@nodiscard
function methods.kerning(self, left, right) end

---Get the width of the provided text when rendered in this font.
---@param self SpriteFont
---@param text string
---@return number
---@nodiscard
function methods.text_width(self, text) end

---Get the height of the provided text when rendered in this font.
---@param self SpriteFont
---@param text string
---@param use_line_gap boolean?
---@return number
---@nodiscard
function methods.text_height(self, text, use_line_gap) end

---Get the size of the provided text when rendered in this font.
---@param self SpriteFont
---@param text string
---@param use_line_gap boolean?
---@return Vec2
---@nodiscard
function methods.text_size(self, text, use_line_gap) end

---Generate a string that transforms `text` and inserts newlines so
---that it wraps inside a container with the provided `width`. The
---amount of lines in the resulting text is returned.
---@param self SpriteFont
---@param width number
---@param text string
---@return string
---@return integer
---@nodiscard
function methods.word_wrap(self, width, text) end

---Draws text using this font.
---@param self SpriteFont
---@param text string
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
function methods.draw_text(self, text, pos, color, mode) end

return module
