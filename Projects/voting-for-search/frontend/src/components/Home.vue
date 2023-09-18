<template>
  <div id="app">
    <h1>Vote na pesquisa</h1>
    <div class="button-container">
      <button @click="handleVote('yes')">Sim</button>
      <button @click="handleVote('no')">Não</button>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      message: '',
    }
  },
  methods: {
    async handleVote(vote) {
      try {
        console.log("Seu voto: ", vote);
      } catch (error) {
        console.error("Erro: ", error );
        this.message = "Erro ao conectar-se ao servidor.";
      }
    }
  }
}
</script>

<style>
#app {
  text-align: center;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  margin-top: 60px;
}

.button-container {
  margin-top: 20px;
}

button {
  font-size: 18px;
  padding: 10px 20px;
  margin: 0 10px;
  background-color: #007bff;
  color: #fff;
  border: none;
  cursor: pointer;
}

button:hover {
  background-color: #0056b3;
}

.message {
  font-size: 24px;
  margin-top: 20px;
}
</style>


