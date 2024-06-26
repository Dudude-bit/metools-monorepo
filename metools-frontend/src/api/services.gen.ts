// This file is auto-generated by @hey-api/openapi-ts

import type { CancelablePromise } from './core/CancelablePromise';
import { OpenAPI } from './core/OpenAPI';
import { request as __request } from './core/request';
import type { ListTasksData, ListTasksResponse, CreateTaskData2, CreateTaskResponse, DeleteAllTasksForUserData, DeleteAllTasksForUserResponse, DeleteTaskByIdForUserData, DeleteTaskByIdForUserResponse, LoginData2, LoginResponse, MeData, MeResponse, SignupData, SignupResponse } from './types.gen';

/**
 * @param data The data for the request.
 * @param data.xApiAuthToken Auth token
 * @returns ResponseListTasks OK
 * @throws ApiError
 */
export const listTasks = (data: ListTasksData): CancelablePromise<ListTasksResponse> => { return __request(OpenAPI, {
    method: 'GET',
    url: '/api/v1/rzd/tasks',
    headers: {
        'X-API-AUTH-TOKEN': data.xApiAuthToken
    },
    errors: {
        401: 'Unauthorized',
        500: 'INTERNAL_SERVER_ERROR'
    }
}); };

/**
 * @param data The data for the request.
 * @param data.xApiAuthToken Auth token
 * @param data.requestBody
 * @returns ResponseCreateTask OK
 * @throws ApiError
 */
export const createTask = (data: CreateTaskData2): CancelablePromise<CreateTaskResponse> => { return __request(OpenAPI, {
    method: 'POST',
    url: '/api/v1/rzd/tasks',
    headers: {
        'X-API-AUTH-TOKEN': data.xApiAuthToken
    },
    body: data.requestBody,
    mediaType: 'application/json',
    errors: {
        400: 'Data is not valid',
        401: 'Unauthorized',
        500: 'INTERNAL_SERVER_ERROR'
    }
}); };

/**
 * @param data The data for the request.
 * @param data.xApiAuthToken Auth token
 * @returns ResponseDeleteAllTasksForUser OK
 * @throws ApiError
 */
export const deleteAllTasksForUser = (data: DeleteAllTasksForUserData): CancelablePromise<DeleteAllTasksForUserResponse> => { return __request(OpenAPI, {
    method: 'DELETE',
    url: '/api/v1/rzd/tasks',
    headers: {
        'X-API-AUTH-TOKEN': data.xApiAuthToken
    },
    errors: {
        401: 'Unauthorized',
        500: 'INTERNAL_SERVER_ERROR'
    }
}); };

/**
 * @param data The data for the request.
 * @param data.taskId Task id
 * @param data.xApiAuthToken Auth token
 * @returns ResponseDeleteTaskByIdForUser OK
 * @throws ApiError
 */
export const deleteTaskByIdForUser = (data: DeleteTaskByIdForUserData): CancelablePromise<DeleteTaskByIdForUserResponse> => { return __request(OpenAPI, {
    method: 'DELETE',
    url: '/api/v1/rzd/tasks/{task_id}',
    path: {
        task_id: data.taskId
    },
    headers: {
        'X-API-AUTH-TOKEN': data.xApiAuthToken
    },
    errors: {
        401: 'Unauthorized',
        404: 'Task not found for user',
        500: 'INTERNAL_SERVER_ERROR'
    }
}); };

/**
 * @param data The data for the request.
 * @param data.requestBody
 * @returns ResponseLogin OK
 * @throws ApiError
 */
export const login = (data: LoginData2): CancelablePromise<LoginResponse> => { return __request(OpenAPI, {
    method: 'POST',
    url: '/api/v1/users/login',
    body: data.requestBody,
    mediaType: 'application/json',
    errors: {
        400: 'Data is not valid'
    }
}); };

/**
 * @param data The data for the request.
 * @param data.xApiAuthToken Auth token
 * @returns ResponseMe OK
 * @throws ApiError
 */
export const me = (data: MeData): CancelablePromise<MeResponse> => { return __request(OpenAPI, {
    method: 'GET',
    url: '/api/v1/users/me',
    headers: {
        'X-API-AUTH-TOKEN': data.xApiAuthToken
    },
    errors: {
        401: 'Unauthorized',
        500: 'Internal server error'
    }
}); };

/**
 * @param data The data for the request.
 * @param data.requestBody
 * @returns ResponseSignUp OK
 * @throws ApiError
 */
export const signup = (data: SignupData): CancelablePromise<SignupResponse> => { return __request(OpenAPI, {
    method: 'POST',
    url: '/api/v1/users/signup',
    body: data.requestBody,
    mediaType: 'application/json',
    errors: {
        400: 'Data is not valid'
    }
}); };