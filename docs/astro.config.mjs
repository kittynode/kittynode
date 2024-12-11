// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightLinksValidator from "starlight-links-validator";
import starlightImageZoom from "starlight-image-zoom";

// https://astro.build/config
export default defineConfig({
  site: "https://kittynode.io",
  integrations: [
    starlight({
      plugins: [starlightLinksValidator(), starlightImageZoom()],
      title: "Kittynode",
      logo: {
        light: "../assets/kittynode-wordmark-light.png",
        dark: "../assets/kittynode-wordmark-dark.png",
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
            {
              label: "Architecture",
              slug: "learn/architecture",
            },
          ],
        },
        {
          label: "Development",
          items: [
            {
              label: "Development guide",
              slug: "development/development-guide",
            },
            { label: "Releases", slug: "development/releases" },
          ],
        },
        {
          label: "Resources",
          items: [
            { label: "Hardware guide", slug: "resources/hardware-guide" },
            {
              label: "Questions and ideas",
              slug: "resources/questions-and-ideas",
            },
          ],
        },
      ],
    }),
  ],
});
