---@meta

---@alias ImageFormat
---     |"grey8"
---     |"grey16"
---     |"grey32f"
---     |"grey_alpha8"
---     |"grey_alpha16"
---     |"grey_alpha32f"
---     |"rgb8"
---     |"rgb16"
---     |"rgb32f"
---     |"rgba8"
---     |"rgba16"
---     |"rgba32f"

---@class Image: ImageMethods

---@class ImageClass: ImageMethods
---@overload fun(width: integer, height: integer, format: ImageFormat): Image
local module = {}

---Create a new image.
---@param width integer
---@param height integer
---@param format ImageFormat? Defaults to `"rgba8"`.
---@return Image
---@nodiscard
function module.new(width, height, format) end

---@class ImageMethods
local methods = {}

---Returns a clone of this image.
---@param self Image
---@return Image
---@nodiscard
function methods.clone(self) end

---The image format.
---@param self Image
---@return ImageFormat?
---@nodiscard
function methods.format(self) end

---Width of the image.
---@param self Image
---@return integer
---@nodiscard
function methods.width(self) end

---Height of the image.
---@param self Image
---@return integer
---@nodiscard
function methods.height(self) end

---Size of the image.
---@param self Image
---@return integer width
---@return integer height
---@nodiscard
function methods.size(self) end

---Creates a clone of this image.
---@param self Image
---@return Image
---@nodiscard
function methods.clone(self) end

---Converts the image to `"rgba"` format and returns it. If the image is already this format, then
---it will just return itself.
---@param self Image
---@return Image
---@nodiscard
function methods.to_rgba8(self) end

-- TODO:
-- ---Returns an iterator over each pixel that yields `(color, x, y)`.
-- ---@param self Image
-- ---@return fun(): integer, integer, integer
-- ---@nodiscard
-- function methods.pixels(self) end

-- TODO:
-- ---Returns an iterator over each pixel in the region that yields `(color, x, y)`.
-- ---@param self Image
-- ---@param x integer
-- ---@param y integer
-- ---@param w integer
-- ---@param h integer
-- ---@return fun(): integer, integer, integer
-- ---@nodiscard
-- function methods.pixels(self, x, y, w, h) end

---Color of the pixel at `(x, y)`.
---@param self Image
---@param x integer
---@param y integer
---@return integer?
---@nodiscard
function methods.get_pixel(self, x, y) end

---Sets the color of the pixel at `(x, y)`.
---@param self Image
---@param x integer
---@param y integer
---@param color integer
function methods.set_pixel(self, x, y, color) end

---Fills the image with the color.
---@param self Image
---@param color integer
function methods.fill(self, color) end

---Fills a region of the image with the color.
---@param self Image
---@param rect Rect
---@param color integer
function methods.fill_rect(self, rect, color) end

---Fills a region of the image with the color.
---@param self Image
---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@param color integer
function methods.fill_at(self, x, y, w, h, color) end

---Draws another image onto this one.
---@param self Image
---@param image Image
---@param dst_x integer
---@param dst_y integer
function methods.draw(self, image, dst_x, dst_y) end

---Draws part of another image onto this one.
---@param self Image
---@param src Image
---@param src_x integer
---@param src_y integer
---@param src_w integer
---@param src_h integer
---@param dst_x integer
---@param dst_y integer
function methods.draw_part(self, src, src_x, src_y, src_w, src_h, dst_x, dst_y) end

---Create a new image that is a portion of this one.
---@param self Image
---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@return Image
---@nodiscard
function methods.sub_image(self, x, y, w, h) end

return module