import { $ } from "bun";
import sharp from "sharp";

const rawPadding = 50;
const rawSize = 512;

// const SHADOW_MARGIN = 40
const SHADOW_BLUR = 35;
// const SHADOW_OFFSET = 6
const SHADOW_OPACITY = 0.3;

const rawIcon = sharp("icon/rawIcon.png");

const { width, height } = await rawIcon.metadata();

if (!width || !height) {
  throw new Error("Width or height not defined");
}

const padding = Math.round((rawPadding / (rawSize - rawPadding * 2)) * width);

const iconMask = Buffer.from(
  `<svg viewBox="0 0 ${width} ${height}"><rect width="${width}" height="${height}" rx="${width / 4}" /></svg>`,
);

const size = padding * 2 + width;

const iconShadow = await sharp(
  Buffer.from(
    `<svg viewBox="0 0 ${size} ${size}"><rect width="${width}" height="${height}" x="${padding}" y="${padding * 1.1}" rx="${width / 4}" fill="rgba(0, 0, 0, ${SHADOW_OPACITY})"
  /></svg>`,
  ),
)
  .blur(SHADOW_BLUR)
  .toBuffer();

await sharp({
  create: {
    width: size,
    height: size,
    channels: 4,
    background: { r: 0, g: 0, b: 0, alpha: 0 },
  },
})
  .composite([
    { input: iconShadow, blend: "multiply" },
    {
      top: padding,
      left: padding,
      input: await rawIcon
        .clone()
        .composite([{ input: iconMask, blend: "dest-in" }])
        .toBuffer(),
      blend: "over",
    },
  ])
  .png()
  .toFile("icon/icon.png");

rawIcon
  .resize(width / 2)
  .png()
  .toFile("icon/smallRawIcon.png");

await $`bunx tauri icon "icon/icon.png"`;
