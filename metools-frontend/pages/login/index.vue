<script setup lang="ts">
import {type InferType, object, string} from "yup";
import type { FormSubmitEvent } from '#ui/types'
import {useUserStore} from "~/stores/user";

const schema = object({
  username: string().required('Required'),
  password: string()
      .min(8, 'Must be at least 8 characters')
      .max(512, "Must be not greater than 512 characters")
      .required('Required')
})
type Schema = InferType<typeof schema>
const user = useUserStore()
async function onSubmit(event: FormSubmitEvent<Schema>) {
  await user.login(event.data.username, event.data.password)
}

const state = reactive({
  username: undefined,
  password: undefined
})
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
    <UFormGroup label="Username" name="username">
      <UInput v-model="state.username" />
    </UFormGroup>

    <UFormGroup label="Password" name="password">
      <UInput v-model="state.password" type="password" />
    </UFormGroup>

    <UButton type="submit">
      Login
    </UButton>
  </UForm>
</template>