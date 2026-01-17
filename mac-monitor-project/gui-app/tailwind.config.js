/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                // macOS System Colors (mapped to CSS variables for dynamic theme support)
                macos: {
                    bg: 'var(--macos-bg)',
                    sidebar: 'var(--macos-sidebar)',
                    divider: 'var(--macos-divider)',
                    text: 'var(--macos-text)',
                    'text-secondary': 'var(--macos-text-secondary)',
                    accent: 'var(--macos-accent)',
                    'accent-hover': 'var(--macos-accent-hover)',
                    border: 'var(--macos-border)',
                }
            },
            fontFamily: {
                sans: [
                    '-apple-system',
                    'BlinkMacSystemFont',
                    '"SF Pro Text"',
                    '"Helvetica Neue"',
                    'sans-serif'
                ],
            },
            boxShadow: {
                'macos-window': '0 20px 68px rgba(0, 0, 0, 0.55)',
                'macos-active': '0 0 0 1px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.1)',
            }
        },
    },
    plugins: [],
}
