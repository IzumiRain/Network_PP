---
slug: shaggy-quail-31
role: button-3
url: https://uiverse.io/icochran10/shaggy-quail-31
keywords: button, menu, navigation, chat, settings, message, green, blue, icon, purple, settings, gear, bell, notification, chat, ai, notify, notifications
license: MIT (uiverse.io)
---

# HTML

```html
<!-- HTML -->
<ul class="w-64 flex flex-col gap-1 border-l border-gray-200 pl-1">
  <li
    class="group w-14 overflow-hidden rounded-lg border-l border-transparent bg-white transition-all duration-500 hover:w-64 hover:border-gray-200 hover:shadow-lg has-[:focus]:w-64 has-[:focus]:shadow-lg"
  >
    <button
      class="peer flex w-full cursor-pointer items-center gap-2.5 px-3 py-2 text-left text-purple-800 transition-all active:scale-95"
    >
      <div class="rounded-lg border-2 border-purple-300 bg-purple-100 p-1">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0"
          ></path>
        </svg>
      </div>
      <div class="font-semibold">Notifications</div>
    </button>
    <div
      class="grid grid-rows-[0fr] overflow-hidden transition-all duration-500 peer-focus:grid-rows-[1fr]"
    >
      <div class="overflow-hidden">
        <ul class="divide-y divide-gray-200 p-4 pt-0">
          <li class="py-2">
            <div class="flex items-center justify-between">
              <button
                class="cursor-pointer font-semibold text-gray-800 hover:text-blue-600"
              >
                Email
              </button>
              <div class="text-sm text-gray-500">2m ago</div>
            </div>
            <div class="text-xs text-gray-500">from Wanye Enterprises</div>
          </li>
          <li class="py-1">
            <div class="flex items-center justify-between">
              <button
                class="cursor-pointer font-semibold text-gray-800 hover:text-blue-600"
              >
                Request
              </button>
              <div class="text-sm text-gray-500">14m ago</div>
            </div>
            <div class="text-xs text-gray-500">from Acme Corporation</div>
          </li>
        </ul>
      </div>
    </div>
  </li>
  <li
    class="group w-14 overflow-hidden rounded-lg border-l border-transparent bg-white transition-all duration-500 hover:w-64 hover:border-gray-200 hover:shadow-lg has-[:focus]:w-64 has-[:focus]:shadow-lg"
  >
    <button
      class="peer flex w-full cursor-pointer items-center gap-2.5 px-3 py-2 text-left text-blue-800 transition-all active:scale-95"
    >
      <div class="rounded-lg border-2 border-blue-300 bg-blue-100 p-1">
        <svg
          class="size-6"
          stroke="currentColor"
          stroke-width="1.5"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"
            stroke-linejoin="round"
            stroke-linecap="round"
          ></path>
          <path
            d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
            stroke-linejoin="round"
            stroke-linecap="round"
          ></path>
        </svg>
      </div>
      <div class="font-semibold">Settings</div>
    </button>
    <div
      class="grid grid-rows-[0fr] overflow-hidden transition-all duration-500 peer-focus:grid-rows-[1fr]"
    >
      <div class="overflow-hidden">
        <ul class="divide-y divide-gray-200 p-4 pt-0">
          <li class="py-2">
            <div class="flex items-center justify-between">
              <button
                class="peer cursor-pointer font-semibold text-gray-800 hover:text-blue-600"
              >
                System Preferences
              </button>
              <div
                class="text-sm text-gray-500 transition-all peer-hover:translate-x-1"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  class="size-4"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="m8.25 4.5 7.5 7.5-7.5 7.5"
                  ></path>
                </svg>
              </div>
            </div>
            <div class="text-xs text-gray-500">Default Settings / Profile</div>
          </li>
          <li class="py-1">
            <div class="group/title flex items-center justify-between">
              <button
                class="peer cursor-pointer font-semibold text-gray-800 hover:text-blue-600"
              >
                Theme
              </button>
              <div
                class="text-sm text-gray-500 transition-all peer-hover:translate-x-1"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  class="size-4"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="m8.25 4.5 7.5 7.5-7.5 7.5"
                  ></path>
                </svg>
              </div>
            </div>
            <div class="text-xs text-gray-500">Light / Dark Mode</div>
          </li>
        </ul>
      </div>
    </div>
  </li>
  <li
    class="group w-14 overflow-hidden rounded-lg border-l border-transparent bg-white transition-all duration-500 hover:w-64 hover:border-gray-200 hover:shadow-lg has-[:focus]:w-64 has-[:focus]:shadow-lg"
  >
    <button
      class="peer flex w-full cursor-pointer items-center gap-2.5 px-3 py-2 text-left text-green-800 transition-all active:scale-95"
    >
      <div class="rounded-lg border-2 border-green-300 bg-green-100 p-1">
        <svg
          class="size-6"
          stroke="currentColor"
          stroke-width="1.5"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M8.625 9.75a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375m-13.5 3.01c0 1.6 1.123 2.994 2.707 3.227 1.087.16 2.185.283 3.293.369V21l4.184-4.183a1.14 1.14 0 0 1 .778-.332 48.294 48.294 0 0 0 5.83-.498c1.585-.233 2.708-1.626 2.708-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0 0 12 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018Z"
            stroke-linejoin="round"
            stroke-linecap="round"
          ></path>
        </svg>
      </div>
      <div class="font-semibold">Chat</div>
    </button>
    <div
      class="grid grid-rows-[0fr] overflow-hidden transition-all duration-500 peer-focus:grid-rows-[1fr]"
    >
      <div class="overflow-hidden">
        <ul class="border-t border-gray-200 p-4 pt-0.5">
          <li class="flex flex-col items-end">
            <div class="text-right text-xs">8:34 AM</div>
            <div
              class="w-40 rounded-lg bg-blue-600/70 px-2 py-1 text-right text-sm text-white"
            >
              Hey TARS, what's your honesty parameter?
            </div>
          </li>
          <li class="flex flex-col items-start">
            <div class="text-right text-xs">8:37 AM</div>
            <div class="w-40 rounded-lg bg-gray-100 px-2 py-1 text-sm">
              90 percent.
            </div>
          </li>
        </ul>
        <div class="relative">
          <input
            class="h-8 w-full rounded-b-lg border border-gray-200 bg-gray-100 pl-2 text-sm"
            placeholder="Reply"
            type="text"
          />
          <button
            class="absolute bottom-0 right-2 top-0 my-auto size-fit cursor-pointer text-blue-600 hover:text-blue-700"
          >
            <svg
              class="size-5"
              stroke="currentColor"
              stroke-width="1.5"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="m15 11.25-3-3m0 0-3 3m3-3v7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                stroke-linejoin="round"
                stroke-linecap="round"
              ></path>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </li>
</ul>
```

# CSS

```css
/* ! tailwindcss v3.4.16 | MIT License | https://tailwindcss.com */*,::after,::before{box-sizing:border-box;border-width:0;border-style:solid;border-color:#e5e7eb}::after,::before{--tw-content:''}:host,html{line-height:1.5;-webkit-text-size-adjust:100%;-moz-tab-size:4;tab-size:4;font-family:ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";font-feature-settings:normal;font-variation-settings:normal;-webkit-tap-highlight-color:transparent}body{margin:0;line-height:inherit}hr{height:0;color:inherit;border-top-width:1px}abbr:where([title]){-webkit-text-decoration:underline dotted;text-decoration:underline dotted}h1,h2,h3,h4,h5,h6{font-size:inherit;font-weight:inherit}a{color:inherit;text-decoration:inherit}b,strong{font-weight:bolder}code,kbd,pre,samp{font-family:ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;font-feature-settings:normal;font-variation-settings:normal;font-size:1em}small{font-size:80%}sub,sup{font-size:75%;line-height:0;position:relative;vertical-align:baseline}sub{bottom:-.25em}sup{top:-.5em}table{text-indent:0;border-color:inherit;border-collapse:collapse}button,input,optgroup,select,textarea{font-family:inherit;font-feature-settings:inherit;font-variation-settings:inherit;font-size:100%;font-weight:inherit;line-height:inherit;letter-spacing:inherit;color:inherit;margin:0;padding:0}button,select{text-transform:none}button,input:where([type=button]),input:where([type=reset]),input:where([type=submit]){-webkit-appearance:button;background-color:transparent;background-image:none}:-moz-focusring{outline:auto}:-moz-ui-invalid{box-shadow:none}progress{vertical-align:baseline}::-webkit-inner-spin-button,::-webkit-outer-spin-button{height:auto}[type=search]{-webkit-appearance:textfield;outline-offset:-2px}::-webkit-search-decoration{-webkit-appearance:none}::-webkit-file-upload-button{-webkit-appearance:button;font:inherit}summary{display:list-item}blockquote,dd,dl,figure,h1,h2,h3,h4,h5,h6,hr,p,pre{margin:0}fieldset{margin:0;padding:0}legend{padding:0}menu,ol,ul{list-style:none;margin:0;padding:0}dialog{padding:0}textarea{resize:vertical}input::placeholder,textarea::placeholder{opacity:1;color:#9ca3af}[role=button],button{cursor:pointer}:disabled{cursor:default}audio,canvas,embed,iframe,img,object,svg,video{display:block;vertical-align:middle}img,video{max-width:100%;height:auto}[hidden]:where(:not([hidden=until-found])){display:none}.absolute{position:absolute}.relative{position:relative}.bottom-0{bottom:0px}.right-2{right:0.5rem}.top-0{top:0px}.my-auto{margin-top:auto;margin-bottom:auto}.flex{display:flex}.grid{display:grid}.size-4{width:1rem;height:1rem}.size-5{width:1.25rem;height:1.25rem}.size-6{width:1.5rem;height:1.5rem}.size-fit{width:-moz-fit-content;width:fit-content;height:-moz-fit-content;height:fit-content}.h-8{height:2rem}.w-14{width:3.5rem}.w-40{width:10rem}.w-64{width:16rem}.w-full{width:100%}.cursor-pointer{cursor:pointer}.grid-rows-\[0fr\]{grid-template-rows:0fr}.flex-col{flex-direction:column}.items-start{align-items:flex-start}.items-end{align-items:flex-end}.items-center{align-items:center}.justify-between{justify-content:space-between}.gap-1{gap:0.25rem}.gap-2\.5{gap:0.625rem}.divide-y > :not([hidden]) ~ :not([hidden]){--tw-divide-y-reverse:0;border-top-width:calc(1px * calc(1 - var(--tw-divide-y-reverse)));border-bottom-width:calc(1px * var(--tw-divide-y-reverse))}.divide-gray-200 > :not([hidden]) ~ :not([hidden]){--tw-divide-opacity:1;border-color:rgb(229 231 235 / var(--tw-divide-opacity, 1))}.overflow-hidden{overflow:hidden}.rounded-lg{border-radius:0.5rem}.rounded-b-lg{border-bottom-right-radius:0.5rem;border-bottom-left-radius:0.5rem}.border{border-width:1px}.border-2{border-width:2px}.border-l{border-left-width:1px}.border-t{border-top-width:1px}.border-blue-300{--tw-border-opacity:1;border-color:rgb(147 197 253 / var(--tw-border-opacity, 1))}.border-gray-200{--tw-border-opacity:1;border-color:rgb(229 231 235 / var(--tw-border-opacity, 1))}.border-green-300{--tw-border-opacity:1;border-color:rgb(134 239 172 / var(--tw-border-opacity, 1))}.border-purple-300{--tw-border-opacity:1;border-color:rgb(216 180 254 / var(--tw-border-opacity, 1))}.border-transparent{border-color:transparent}.bg-blue-100{--tw-bg-opacity:1;background-color:rgb(219 234 254 / var(--tw-bg-opacity, 1))}.bg-blue-600\/70{background-color:rgb(37 99 235 / 0.7)}.bg-gray-100{--tw-bg-opacity:1;background-color:rgb(243 244 246 / var(--tw-bg-opacity, 1))}.bg-green-100{--tw-bg-opacity:1;background-color:rgb(220 252 231 / var(--tw-bg-opacity, 1))}.bg-purple-100{--tw-bg-opacity:1;background-color:rgb(243 232 255 / var(--tw-bg-opacity, 1))}.bg-white{--tw-bg-opacity:1;background-color:rgb(255 255 255 / var(--tw-bg-opacity, 1))}.p-1{padding:0.25rem}.p-4{padding:1rem}.px-2{padding-left:0.5rem;padding-right:0.5rem}.px-3{padding-left:0.75rem;padding-right:0.75rem}.py-1{padding-top:0.25rem;padding-bottom:0.25rem}.py-2{padding-top:0.5rem;padding-bottom:0.5rem}.pl-1{padding-left:0.25rem}.pl-2{padding-left:0.5rem}.pt-0{padding-top:0px}.pt-0\.5{padding-top:0.125rem}.text-left{text-align:left}.text-right{text-align:right}.text-sm{font-size:0.875rem;line-height:1.25rem}.text-xs{font-size:0.75rem;line-height:1rem}.font-semibold{font-weight:600}.text-blue-600{--tw-text-opacity:1;color:rgb(37 99 235 / var(--tw-text-opacity, 1))}.text-blue-800{--tw-text-opacity:1;color:rgb(30 64 175 / var(--tw-text-opacity, 1))}.text-gray-500{--tw-text-opacity:1;color:rgb(107 114 128 / var(--tw-text-opacity, 1))}.text-gray-800{--tw-text-opacity:1;color:rgb(31 41 55 / var(--tw-text-opacity, 1))}.text-green-800{--tw-text-opacity:1;color:rgb(22 101 52 / var(--tw-text-opacity, 1))}.text-purple-800{--tw-text-opacity:1;color:rgb(107 33 168 / var(--tw-text-opacity, 1))}.text-white{--tw-text-opacity:1;color:rgb(255 255 255 / var(--tw-text-opacity, 1))}.transition-all{transition-property:all;transition-timing-function:cubic-bezier(0.4, 0, 0.2, 1);transition-duration:150ms}.duration-500{transition-duration:500ms}.hover\:w-64:hover{width:16rem}.hover\:border-gray-200:hover{--tw-border-opacity:1;border-color:rgb(229 231 235 / var(--tw-border-opacity, 1))}.hover\:text-blue-600:hover{--tw-text-opacity:1;color:rgb(37 99 235 / var(--tw-text-opacity, 1))}.hover\:text-blue-700:hover{--tw-text-opacity:1;color:rgb(29 78 216 / var(--tw-text-opacity, 1))}.hover\:shadow-lg:hover{--tw-shadow:0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);--tw-shadow-colored:0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color);box-shadow:var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)}.active\:scale-95:active{--tw-scale-x:.95;--tw-scale-y:.95;transform:translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))}.peer:hover ~ .peer-hover\:translate-x-1{--tw-translate-x:0.25rem;transform:translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))}.peer:focus ~ .peer-focus\:grid-rows-\[1fr\]{grid-template-rows:1fr}.has-\[\:focus\]\:w-64:has(:focus){width:16rem}.has-\[\:focus\]\:shadow-lg:has(:focus){--tw-shadow:0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);--tw-shadow-colored:0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color);box-shadow:var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)}
```
