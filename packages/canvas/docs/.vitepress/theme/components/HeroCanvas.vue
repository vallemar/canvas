<template>
  <DefaultTheme.Layout>
    <template #home-hero-image>
      <div>
        <div class="h-[300px] w-[200px] rounded-3xl">
          <canvas id=c ref="canvas" class="h-[350px] w-[240px] rounded-xl"></canvas>

        </div>
      </div>
    </template>
  </DefaultTheme.Layout>
</template>
<script setup>
import DefaultTheme from 'vitepress/theme'

import { onMounted, ref } from "vue"
const canvas = ref(null)
onMounted(() => {
  /*  const canvasEl = document.querySelector(".VPHero.VPHomeHero .container");
   canvasEl.append(canvas.value); */

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

    ctx.fillStyle = 'rgba(0, 0, 0, .04)';
    ctx.fillRect(0, 0, w, h);
    ctx.shadowBlur = 0;

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
})
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
</style>