import { LOGIN_ON_CHANGE, LOGIN_ON_SUBMIT, LOGIN_ON_SUBMIT_SUCCESS, LOGIN_ON_SUBMIT_ERROR } from "./LoginActions"

export interface LoginState {
    email: string
    password: string
    loading: boolean
    token?: string
}

const initState: LoginState = {
    email: "",
    password: "",
    loading: false,
    token: undefined,
}

const LoginReducer = (state: LoginState = initState, action: any) => {
    switch (action.type) {
        case LOGIN_ON_CHANGE: return { ...state, [action.field]: action.value }
        case LOGIN_ON_SUBMIT: return { ...state, loading: true, error: undefined }
        case LOGIN_ON_SUBMIT_SUCCESS: return { ...state, token: action.token, loading: false, email: "", password: "" }
        case LOGIN_ON_SUBMIT_ERROR: return { ...state, error: action.error, loading: false }
        default: return state
    }
}

export default LoginReducer
