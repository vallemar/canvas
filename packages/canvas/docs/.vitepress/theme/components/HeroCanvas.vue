<template>
  <DefaultTheme.Layout>
    <template #home-hero-info>
      <div class="w-full text-center flex flex-col justify-center items-center relative" style="min-height: 350px;">
        <div class="text-center">
          <h1 class="name"><span class="clip">NativeScript Canvas</span></h1>
          <p class="text">Native Canvas for NativeScript</p><!--v-if-->
          <div class="actions flex justify-center">
            <div class="action"><a class="VPButton medium brand" href="/installation">Get Started</a></div>
            <div class="action"><a class="VPButton medium alt" href="/examples">Examples</a></div>
          </div>
        </div>

        <div class="h-[350px] w-full rounded-3xl absolute left-0 top-0" style="z-index: -1;">
          <canvas id=c ref="canvas" class="h-[350px] w-full rounded-xl"></canvas>
        </div>
      </div>
    </template>
  </DefaultTheme.Layout>
</template>
<script setup>
import DefaultTheme from 'vitepress/theme'
import { onContentUpdated } from "vitepress"
import { onMounted, onUpdated, ref } from "vue"
const canvas = ref(null)
onContentUpdated((args) => {
  if (window.location.pathname === "/") {
    runAnimation();
  }
})
onMounted(() => {
  runAnimation();
})

function runAnimation() {
  var w = c.width = window.innerWidth,
    h = c.height = window.innerHeight,
    ctx = c.getContext('2d'),

    spawnProb = 1,
    numberOfMoves = [8, 16], //[min, max]
    distance = [50, 400],
    attenuator = 30,
    timeBetweenMoves = [4, 10],

    lines = [],
    frame = (Math.random() * 360) | 0;

  ctx.lineWidth = 1;

  function rand(ar) {
    return Math.random() * (ar[1] - ar[0]) + ar[0];
  }
  function Line() {
    this.x = Math.random() * w;
    this.y = Math.random() * h;
    this.last = {};
    this.target = {};
    this.totalMoves = rand(numberOfMoves);
    this.move = 0;
    this.timeBetweenMoves = rand(timeBetweenMoves);
    this.timeSpentThisMove = this.timeBetweenMoves;
    this.distance = rand(distance);

    this.color = 'hsl(hue, 80%, 50%)'.replace('hue', frame % 360);
  }
  Line.prototype.use = function () {
    ++this.timeSpentThisMove;
    if (this.timeSpentThisMove >= this.timeBetweenMoves) {
      ++this.move;
      this.timeSpentThisMove = 0;

      var rad = Math.random() * 2 * Math.PI;
      this.target.x = this.x + Math.cos(rad) * this.distance;
      this.target.y = this.y + Math.sin(rad) * this.distance;
    }

    this.last.x = this.x;
    this.last.y = this.y;

    this.x += (this.x - this.target.x) / attenuator;
    this.y += (this.y - this.target.y) / attenuator;

    ctx.strokeStyle = ctx.shadowColor = this.color;
    ctx.beginPath();
    ctx.moveTo(this.last.x, this.last.y);
    ctx.lineTo(this.x, this.y);
    ctx.stroke();
  }

  function anim() {
    window.requestAnimationFrame(anim);

    frame += 1.5;
    //18171c
    ctx.shadowBlur = 0;
    //ctx.fillStyle = 'rgba(32 33 38, .04)';

    ctx.fillStyle = 'rgba(0, 0, 0, .2)';
    ctx.fillRect(0, 0, w, h);
    //ctx.shadowBlur = 0;

    if (Math.random() < spawnProb) lines.push(new Line);

    for (var i = 0; i < lines.length; ++i) {
      lines[i].use();

      if (lines[i].move >= lines[i].totalMoves) {
        lines.splice(i, 1);
        --i;
      }
    }
  }
  anim();
}
</script>
<style>
.VPHero .image-bg {
  display: none;
}

.VPHero .image-container {
  display: flex;
  width: 100%;
  justify-content: center;
  align-items: center;
  flex-shrink: 0;
}

.clip {
  background: var(--vp-home-hero-name-background);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: var(--vp-home-hero-name-color);
}

.name {
  color: var(--vp-home-hero-name-color);
}

.name,
.text {
  max-width: 392px;
  letter-spacing: -0.4px;
  line-height: 40px;
  font-size: 32px;
  font-weight: 700;
  white-space: pre-wrap;
}

@media (min-width: 640px) {

  .name,
  .text {
    max-width: 576px;
    line-height: 56px;
    font-size: 48px;
  }

}

@media (min-width: 960px) {

  .name,
  .text {
    line-height: 64px;
    font-size: 56px;
  }
}

.actions {
  display: flex;
  flex-wrap: wrap;
  margin: -6px;
  padding-top: 24px;
}

.action {
  flex-shrink: 0;
  padding: 6px;
}

.VPButton.brand {
  border-color: var(--vp-button-brand-border);
  color: var(--vp-button-brand-text);
  background-color: var(--vp-button-brand-bg);
}


.VPButton.medium {
  border-radius: 20px;
  padding: 0 20px;
  line-height: 38px;
  font-size: 14px;
}

.VPButton {
  display: inline-block;
  border: 1px solid transparent;
  text-align: center;
  font-weight: 600;
  white-space: nowrap;
  transition: color 0.25s, border-color 0.25s, background-color 0.25s;
}

.VPButton.alt {
  border-color: var(--vp-button-alt-border);
  color: var(--vp-button-alt-text);
  background-color: var(--vp-button-alt-bg);
}
</style>