import * as React from "react"
import * as ReactDOM from "react-dom"
import { Provider } from "react-redux"
import { createStore, compose, combineReducers, applyMiddleware } from "redux"
import createHistory from "history/createBrowserHistory"
import { ConnectedRouter, routerReducer, routerMiddleware, push } from "react-router-redux"
import { Route } from "react-router"

import LoginReducer, { LoginState } from "./login/LoginReducer"
import LoginContainer from "./login/LoginContainer"


const history = createHistory()
const middleware = routerMiddleware(history)

const reducers = combineReducers({
    login: LoginReducer,
    router: routerReducer
})

export let store = createStore(
    reducers,
    compose(
        applyMiddleware(middleware),
        (window as any).__REDUX_DEVTOOLS_EXTENSION__ && (window as any).__REDUX_DEVTOOLS_EXTENSION__()
    )
)

ReactDOM.render(
    <Provider store={store}>
        <ConnectedRouter history={history}>
            <div>
                <Route exact path="/" component={LoginContainer}/>
            </div>
        </ConnectedRouter>
    </Provider>,
    document.getElementById("app")
)