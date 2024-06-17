import { defineStore } from "pinia";
import { login as apiLogin, me as apiMe } from "~/src/api";

export const useUserStore = defineStore("user", {
  state: () => {
    return { username: "", email: "" };
  },
  actions: {
    logout() {
      this.$patch({
        username: "",
        email: "",
      });
      localStorage.removeItem("token");
    },

    async login(username: string, password: string) {
      const dataLogin = await apiLogin({
        requestBody: {
          username: username,
          password: password,
        },
      });
      localStorage.setItem("token", dataLogin.data);
      const dataMe = await apiMe({
        xApiAuthToken: dataLogin.data,
      });
      this.$patch({
        username: dataMe.data.username,
        email: dataMe.data.email,
      });
    },
  },
});
