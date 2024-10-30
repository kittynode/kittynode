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
      components: {
        Footer: "./src/components/overrides/Footer.astro",
      },
      customCss: ["./src/styles/custom.css"],
      favicon: "/images/favicon.ico",
      social: {
        github: "https://github.com/kittynode",
        discord: "https://discord.kittynode.io",
        "x.com": "https://x.com/kittynode",
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
              label: "Architecture",
              slug: "contribute/architecture",
            },
            {
              label: "Development guide",
              slug: "contribute/development-guide",
            },
          ],
        },
      ],
    }),
  ],
});
