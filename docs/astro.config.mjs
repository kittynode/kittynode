// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: "Kittynode Docs",
      customCss: ["./src/styles/custom.css"],
      social: {
        github: "https://github.com/kittynode/kittynode",
      },
      sidebar: [
        {
          label: "Concepts",
          items: [
            { label: "What is Kittynode?", slug: "concepts/what-is-kittynode" },
          ],
        },
      ],
    }),
  ],
});
