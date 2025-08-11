export class TrickFilmPlayer {
  constructor(graphic, trickFilm, trickFilmRegistion) {
    this.graphic = graphic;
    this.entity = graphic.entity;
    this.paused = false;
    this.speed = 1.0;
    this.trickFilm = trickFilm;
    this.trickFilmRegistion = trickFilmRegistion;
    this.server = simpleWarfareCli.trickFilmPlayerServer;
    this.registions = new Object();

    for (const registion of trickFilmRegistion) {
      this.registions[registion] = registion;
    }
  }

  start(registion) {
    this.server.start(this,this.graphic.realParentPath, registion);
  }

  play(registion) {
    this.server.play(this,this.graphic.realParentPath, registion);
  }
}
