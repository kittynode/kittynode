// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  site: "https://kittynode.io",
  integrations: [
    starlight({
      title: "Kittynode",
      logo: {
        light: "./src/assets/kittynode-light.png",
        dark: "./src/assets/kittynode-dark.png",
        replacesTitle: true,
      },
      editLink: {
        baseUrl: "https://github.com/kittynode/kittynode/edit/main/docs/",
      },
      customCss: ["./src/styles/custom.css"],
      favicon: "/images/favicon.ico",
      social: {
        github: "https://github.com/kittynode",
        youtube: "https://youtube.com/@kittynode",
      },
      sidebar: [
        {
          label: "Learn",
          items: [
            { label: "What is Kittynode?", slug: "learn/what-is-kittynode" },
          ],
        },
        {
          label: "Contribute",
          items: [
            {
              label: "Contribute to Kittynode",
              slug: "contribute/contribute-to-kittynode",
            },
          ],
        },
      ],
    }),
  ],
});
