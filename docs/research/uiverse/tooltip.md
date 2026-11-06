---
slug: spicy-kangaroo-36
role: tooltip
url: https://uiverse.io/kelvin_9865/spicy-kangaroo-36
keywords: tooltip, rounded, button, black, white, minimal, centered, tooltip, hover
license: MIT (uiverse.io)
---

# HTML

```html
<!-- HTML -->
<div class="tooltip-container">
  <button class="tooltip-trigger">Hover Me</button>
  <div class="tooltip">This is a 3D glassmorphic tooltip!</div>
</div>
```

# CSS

```css
/* 3D Glassmorphic tooltip by kelvin */

body {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  background: linear-gradient(to right, #000000, #06273d);
  font-family: Arial, sans-serif;
  margin: 0;
}

.tooltip-container {
  position: relative;
  display: inline-block;
}

.tooltip {
  position: absolute;
  bottom: 120%;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
  padding: 15px 20px;
  border-radius: 15px;
  border: 1px solid rgba(255, 255, 255, 0.3);
  backdrop-filter: blur(12px);
  font-size: 14px;
  text-align: center;
  box-shadow:
    0 8px 20px rgba(0, 0, 0, 0.4),
    inset 0 0 10px rgba(255, 255, 255, 0.3);
  white-space: nowrap;
  visibility: hidden;
  opacity: 0;
  transition:
    opacity 0.4s ease,
    transform 0.4s ease;
}

.tooltip::after {
  content: "";
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border-width: 8px;
  border-style: solid;
  border-color: rgba(255, 255, 255, 0.3) transparent transparent transparent;
  filter: drop-shadow(0px 2px 4px rgba(0, 0, 0, 0.3));
}

.tooltip-container:hover .tooltip {
  visibility: visible;
  opacity: 1;
  transform: translateX(-50%) translateY(-15px);
}

.tooltip-container:hover .tooltip::after {
  border-color: rgba(255, 255, 255, 0.6) transparent transparent transparent;
}

.tooltip-trigger {
  padding: 15px 30px;
  color: #fff;
  font-size: 16px;
  text-transform: uppercase;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 10px;
  backdrop-filter: blur(10px);
  cursor: pointer;
  transition:
    background 0.3s ease,
    transform 0.3s ease;
}

.tooltip-trigger:hover {
  background: linear-gradient(45deg, #4facfe, #00f2fe);
  transform: translateY(-3px);
  box-shadow: 0 6px 15px rgba(0, 255, 255, 0.5);
}
```
