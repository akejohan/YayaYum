<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 400" width="200" height="200" role="img" aria-labelledby="title desc">
    <title id="title">Gullig skål med nudlar</title>
    <desc id="desc">Animerad SVG: nudlar, ånga som stiger och wokpinnar som petar</desc>
  
    <style>
      .bowl { fill: #FFF6E8; stroke: #E6B87A; stroke-width: 4; }
      .rim { fill: #FFD8A8; stroke: #E6B87A; stroke-width: 3; }
      .noodle { fill: none; stroke: #F6A623; stroke-width: 10; stroke-linecap: round; }
      .chop { stroke: #5A3E36; stroke-width: 8; stroke-linecap: round; }
      .steam { fill: none; stroke: #ffffff; stroke-width: 5; stroke-linecap: round; opacity: 0.8; }
  
      /* Noddle wiggle */
      @keyframes wobble {
        0% { transform: rotate(-3deg); }
        50% { transform: rotate(3deg); }
        100% { transform: rotate(-3deg); }
      }
      /* Chopstick tap */
      @keyframes tap {
        0% { transform: rotate(-20deg); }
        50% { transform: rotate(-10deg); }
        100% { transform: rotate(-20deg); }
      }
      /* Steam rise */
      @keyframes rise {
        0% { transform: translateY(8px); opacity: 0; }
        10% { opacity: 0.6; }
        100% { transform: translateY(-30px); opacity: 0; }
      }
  
      .noodles-group { transform-box: fill-box; transform-origin: 50% 45%; animation: wobble 2.2s ease-in-out infinite; }
      .chopstick-group { transform-box: fill-box; transform-origin: 60% 28%; animation: tap 1.6s ease-in-out infinite; }
      .steam1 { animation: rise 2.2s ease-in infinite; animation-delay: 0s; }
      .steam2 { animation: rise 2.8s ease-in infinite; animation-delay: 0.6s; }
      .steam3 { animation: rise 3.4s ease-in infinite; animation-delay: 1.1s; }
    </style>
  
    <!-- Background circle for a soft look -->
    <rect width="100%" height="100%" fill="transparent"/>
  
    <!-- Bowl -->
    <g transform="translate(0,40)">
      <ellipse cx="200" cy="260" rx="135" ry="35" class="rim"/>
      <path d="M70 260 Q200 330 330 260 Q300 360 100 360 Z" class="bowl"/>
    </g>
  
    <!-- Nudlar (stiliserade kurvor) -->
    <g class="noodles-group" transform="translate(0,20)">
      <path class="noodle" d="M120 230 C150 200 250 200 280 230" />
      <path class="noodle" d="M130 250 C160 230 240 230 270 250" />
      <path class="noodle" d="M115 215 C150 200 240 210 290 220" />
      <path class="noodle" d="M140 210 C170 190 230 190 260 210" />
      <!-- En liten klimp "topping" -->
      <circle cx="200" cy="230" r="6" fill="#D94E2A"/>
    </g>
  
    <!-- Wokpinnar -->
    <g class="chopstick-group" transform="translate(0,10)">
      <line class="chop" x1="265" y1="160" x2="360" y2="70" />
      <line class="chop" x1="240" y1="170" x2="335" y2="85" />
      <!-- små animationer: en bit nudel mellan pinnarna -->
      <path d="M250 165 C260 158 275 155 285 150" stroke="#F6A623" stroke-width="6" fill="none" stroke-linecap="round"/>
    </g>
  
    <!-- Ånga -->
    <g stroke-linecap="round" stroke-linejoin="round">
      <path class="steam steam1" d="M190 130 C185 110 175 100 170 90" stroke-opacity="0.9"/>
      <path class="steam steam2" d="M210 140 C205 120 195 110 190 100" stroke-opacity="0.7"/>
      <path class="steam steam3" d="M170 145 C165 130 155 120 150 110" stroke-opacity="0.6"/>
    </g>
  
    <!-- Ögon (gulligt ansikte på skålen) -->
    <g transform="translate(0,40)">
      <circle cx="170" cy="300" r="5" fill="#5A3E36"/>
      <circle cx="230" cy="300" r="5" fill="#5A3E36"/>
      <path d="M170 310 Q200 325 230 310" stroke="#5A3E36" stroke-width="3" fill="none" stroke-linecap="round"/>
    </g>
  
  </svg>
  