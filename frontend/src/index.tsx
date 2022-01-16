import React from 'react';
import ReactDOM from 'react-dom';
import { ApolloClient, ApolloLink, ApolloProvider, createHttpLink, gql, InMemoryCache } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';

import './index.css';
import App from './components/App';
import reportWebVitals from './reportWebVitals';
import { AUTH_TOKEN } from './constant';
import { RecoilRoot } from 'recoil';

const httpLink = createHttpLink({ uri: 'http://localhost:4000/' });

const authLink = setContext((_, { headers }) => {
  // get the authentication token from local storage if it exists
  const token = localStorage.getItem(AUTH_TOKEN);
  // return the headers to the context so httpLink can read them
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : "",
    },
  }
});

const client = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache()
});

ReactDOM.render(
  <React.StrictMode>
    <RecoilRoot>
    <ApolloProvider client={client}>
      <App />
    </ApolloProvider>
    </RecoilRoot>
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();