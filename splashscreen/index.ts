import autoprefixer from "autoprefixer";
import cssnanoPlugin from "cssnano";
import postcss from "postcss";
import tailwind from "tailwindcss";

import SplashScreen from "./splashscreen";

const plugins = [tailwind("./tailwind.config.js"), autoprefixer(), cssnanoPlugin()];

const source = "./tailwind.css";
const cssSource = await Bun.file(source).text();

const processedCss = await postcss(plugins).process(cssSource, {
  from: source,
  map: false,
});

const html = SplashScreen(processedCss.css);

await Bun.write("../static/splashscreen.html", html.toString());
