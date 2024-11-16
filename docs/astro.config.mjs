// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightLinksValidator from 'starlight-links-validator'
import starlightImageZoom from 'starlight-image-zoom'

// https://astro.build/config
export default defineConfig({
  site: "https://kittynode.io",
  integrations: [
    starlight({
      plugins: [starlightLinksValidator(), starlightImageZoom()],
      title: "Kittynode",
      logo: {
        light: "../assets/kittynode-light.png",
        dark: "../assets/kittynode-dark.png",
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
        youtube: "https://youtube.com/@kittynode",
      },
      sidebar: [
        {
          label: "Learn",
          items: [
            { label: "What is Kittynode?", slug: "learn/what-is-kittynode" },
            { label: "Why Kittynode?", slug: "learn/why-kittynode" },
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
              label: "Open questions",
              slug: "contribute/questions",
            },
          ],
        },
        {
          label: "Development",
          items: [
            { label: "Development guide", slug: "development/development-guide" },
            { label: "Project tracking", slug: "development/project-tracking" },
            { label: "Releases", slug: "development/releases" },
          ],
        },
        {
          label: "Resources",
          items: [
            { label: "Hardware guide", slug: "resources/hardware-guide" },
          ],
        },
      ],
    }),
  ],
});
