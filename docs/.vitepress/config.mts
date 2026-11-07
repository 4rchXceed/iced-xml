import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  srcDir: "./",
  // base: "/_private/iced-xml-docs/",

  title: "Iced-XML Documentation",
  description: "This project aims to simplify A LOT iced, by combining the speed of Rust & iced.rs with the simplicity of XML Trees to create objects (like HTML), CSS-like stying and DOM-like object manipulation. This project is absolutely not finished. If you're interested, watch this repo for updates.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'User Guide', link: '/user-guide' },
      { text: 'Tutorial', link: '/tutorial' }
    ],

    sidebar: [
      {
        text: "Home",
        link: "/",
        items: [
          { text: "Intro", link: "/INTRO.md" }
        ]
    },
      {
        text: 'Tutorial',
        link: '/tutorial/QUICK_START.md',
        items: [
          { text: 'Quick Start', link: '/tutorial/QUICK_START.md' },
          { text: 'UI Setup', link: '/tutorial/UI_SETUP.md' },
          { text: 'TodoListWindow Setup', link: '/tutorial/1ST_WINDOW_SETUP.md' },
          { text: 'TodoListWindow Elements', link: '/tutorial/1ST_WINDOW_XML.md' },
        ]
      },
      {
        text: 'User Guide',
        link: '/user-guide/',
        items: [
          { text: 'App State', link: '/user-guide/APP_STATE.md' },
          { text: 'Create Window Function', link: '/user-guide/CREATE_WINDOW_FN.md' },
          { text: 'A Window\'s Base', link: '/user-guide/WINDOW_BASE.md' },
          { text: 'The WindowId', link: '/user-guide/WINDOW_ID.md' },
          { text: 'The Window params', link: '/user-guide/WINDOW_PARAMS.md' },
          { text: 'The XML Syntax', link: '/user-guide/XML_SYNTAX.md' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/4rchXceed/iced-xml' }
    ]
  }
})
