---@meta

---@class (exact) SubTexture: SubTextureMethods

---@class SubTextureClass: SubTextureMethods
local module = {}

---@class SubTextureMethods
local methods = {}

---Create a new subtexture from the rectangular sub-region of a texture's pixels.
---You can also provide a rendering offset and virtual size for the subtexture.
---@param texture Texture
---@param rect Rect
---@param offset Vec2?
---@param size Vec2?
function module.new(texture, rect, offset, size) end

-- ---The subtexture's texture.
-- ---@param self SubTexture
-- ---@return Texture
-- ---@nodiscard
-- function methods.texture(self) end

---The subtexture's source rectangle.
---@param self SubTexture
---@return Rect
---@nodiscard
function methods.rect(self) end

---The subtexture's render offset.
---@param self SubTexture
---@return Vec2
---@nodiscard
function methods.offset(self) end

---The subtexture's virtual size.
---@param self SubTexture
---@return Vec2
---@nodiscard
function methods.size(self) end

---The subtexture's texture coordinates.
---@param self SubTexture
---@return Vec2
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function methods.coords(self) end

return module