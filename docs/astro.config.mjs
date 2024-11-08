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
              label: "Development guide",
              slug: "contribute/development-guide",
            },
            {
              label: "Questions",
              slug: "contribute/questions",
            },
          ],
        },
        {
          label: "Resources",
          items: [
            { label: "Hardware", slug: "resources/hardware" },
          ],
        },
      ],
    }),
  ],
});
