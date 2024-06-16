import { defineStore } from 'pinia'
import {login as apiLogin, me as apiMe} from "~/src/api";

export const useUserStore = defineStore('user', {
    state: () => {
        return { username: undefined,  email: undefined, token: undefined }
    },
    actions: {
        logout() {
            this.$patch({
                username: undefined,
                email: undefined,
                token: undefined
            })

        },

        async login(username: string, password: string) {
            const dataLogin = await apiLogin({
                requestBody: {
                    username: username,
                    password: password
                }
            })
            const dataMe = await apiMe(
                {
                    xApiAuthToken: dataLogin.data,
                }
            )
            this.$patch({
                username: dataMe.data.username,
                email: dataMe.data.email,
                token: dataLogin.data,
            })
        }
    },
})