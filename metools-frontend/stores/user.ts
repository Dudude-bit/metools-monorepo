// stores/counter.js
import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
    state: (): {firstName: string, lastName: string} => {
        return { firstName: '', lastName: ''}
    },
    getters: {
        FirstName(state): string {
            return state.firstName ? state.firstName : ''
        },
        LastName(state): string {
            return state.lastName ? state.lastName : ''
        }
    },
    actions: {
        setFirstName(firstName: string): void {
            this.firstName = firstName
        },
        setLastName(lastName: string): void {
            this.lastName = lastName
        }
    }
})