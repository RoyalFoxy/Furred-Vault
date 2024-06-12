export default function SplashScreen(css: string) {
  return (
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Splash screen</title>

        <style>{css}</style>
      </head>
      <body class="bg-[#FFF4E1]">
        <img src="/smallRawIcon.png" class="h-screen w-screen animate-fade-in" />
        <div
          class={`
            fixed bottom-0.5 left-0 flex w-screen items-center justify-center
            space-x-1
          `}
        >
          <div
            class={`
              h-2 w-1 animate-bounce rounded-full bg-[#35191F]

              [animation-delay:-0.3s]
            `}
          />
          <div
            class={`
              h-2 w-1 animate-bounce rounded-full bg-[#35191F]

              [animation-delay:-0.15s]
            `}
          />
          <div class="h-2 w-1 animate-bounce rounded-full bg-[#35191F]" />
        </div>
      </body>
    </html>
  );
}
