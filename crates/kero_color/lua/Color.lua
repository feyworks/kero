---@meta

---@alias Color integer

---@class ColorModule
local Color = {}

---`0x00000000`
---@return Color
---@nodiscard
function Color.transparent() end

---`0x000000ff`
---@return Color
---@nodiscard
function Color.black() end

---`0xffffffff`
---@return Color
---@nodiscard
function Color.white() end

---`0xff0000ff`
---@return Color
---@nodiscard
function Color.red() end

---`0x00ff00ff`
---@return Color
---@nodiscard
function Color.green() end

---`0x0000ffff`
---@return Color
---@nodiscard
function Color.blue() end

---`0xffff00ff`
---@return Color
---@nodiscard
function Color.yellow() end

---`0x00ffffff`
---@return Color
---@nodiscard
function Color.cyan() end

---`0xff00ffff`
---@return Color
---@nodiscard
function Color.fuchsia() end

---Pack channels into a color.
---@param r integer `0-255`
---@param g integer `0-255`
---@param b integer `0-255`
---@param a integer? `0-255`
---@return Color
---@nodiscard
function Color.pack(r, g, b, a) end

---Pack floating-point channels into a color.
---@param r number `0-1`
---@param g number `0-1`
---@param b number `0-1`
---@param a number `0-1`
---@return Color
---@nodiscard
function Color.packf(r, g, b, a) end

---Unpack a color into its channels.
---@param color Color
---@return integer red `0-255`
---@return integer green `0-255`
---@return integer blue `0-255`
---@return integer alpha `0-255`
---@nodiscard
function Color.unpack(color) end

---Unpack a color into its channels.
---@param color Color
---@return number red `0-1`
---@return number green `0-1`
---@return number blue `0-1`
---@return number alpha `0-1`
---@nodiscard
function Color.unpackf(color) end

---Get a color's red channel.
---@param color Color
---@return integer
---@nodiscard
function Color.get_r(color) end

---Get a color's green channel (`0-255`).
---@param color Color
---@return integer
---@nodiscard
function Color.get_g(color) end

---Get a color's blue channel (`0-255`).
---@param color Color
---@return integer
---@nodiscard
function Color.get_b(color) end

---Get a color's alpha channel (`0-255`).
---@param color Color
---@return integer
---@nodiscard
function Color.get_a(color) end

---Multiply the two colors.
---@param a Color
---@param b Color
---@return Color
---@nodiscard
function Color.mul(a, b) end

---Multiplies the color by the normalized alpha value.
---@param color Color
---@param alpha number `0-1`
---@return Color
---@nodiscard
function Color.mul_a(color, alpha) end

---Sum the two colors.
---@param a Color
---@param b Color
---@return Color
---@nodiscard
function Color.add(a, b) end

---Get the difference between two colors.
---@param a Color
---@param b Color
---@return Color
---@nodiscard
function Color.sub(a, b) end

---Create a color represented by hue, saturation, and lightness.
---@param hue number (`0-1`) The color’s hue, representing 0-360º on the color wheel.
---@param saturation number (`0-1`) The color’s saturation, from 0 (greyscale) to 1 (full saturation).
---@param lightness number (`0-1`) The color’s lightness, from 0 (black) to 1 (white).
---@return Color
---@nodiscard
function Color.hsl(hue, saturation, lightness) end

---Get the hue, saturation, and lightness of a color.
---@param color Color
---@return number hue
---@return number saturation
---@return number lightness
---@nodiscard
function Color.to_hsl(color) end

---Create a color represented by hue, saturation, and value.
---@param hue number (`0-1`) The color’s hue, representing 0-360º on the color wheel.
---@param saturation number (`0-1`) The color’s saturation, from 0 (greyscale) to 1 (full saturation).
---@param value number (`0-1`) The color’s value, from 0 (black) to 1 (full color value).
---@return Color
---@nodiscard
function Color.hsv(hue, saturation, value) end

---Get the hue, saturation, and value of a color.
---@param color Color
---@return number hue
---@return number saturation
---@return number value
---@nodiscard
function Color.to_hsv(color) end

---Create a color from its [Oklab](https://bottosson.github.io/posts/oklab) components.
---@param l number
---@param a number
---@param b number
---@return Color
---@nodiscard
function Color.oklab(l, a, b) end

---Get the Oklab](https://bottosson.github.io/posts/oklab) components of the color.
---@param color Color
---@return number l
---@return number a
---@return number b
---@nodiscard
function Color.to_oklab(color) end

---Lerp between two colors by a factor of `t`.
---@param from Color
---@param to Color
---@param t number `0-1`
---@return Color
---@nodiscard
function Color.lerp(from, to, t) end

return Color