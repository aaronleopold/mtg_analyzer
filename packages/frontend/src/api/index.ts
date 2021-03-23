import axios from 'axios';

// TODO: access .env vars here!!
// TODO: add /api suffix
const api = axios.create({
  baseURL: 'http://localhost:8080/',
});

type Method = 'get' | 'delete' | 'post' | 'put';

const apiRequest = (method: Method, url: string, request?: any) => {
  const headers = {
    authorization: '',
  };
  //using the axios instance to perform the request that received from each http method
  return api({
    method,
    url,
    data: request,
    headers,
  })
    .then((res) => {
      return Promise.resolve(res.data);
    })
    .catch((err) => {
      return Promise.reject(err);
    });
};

// function to execute the http get request
const get = (url: string, request?: any) => apiRequest('get', url, request);

// function to execute the http delete request
const deleteRequest = (url: string, request?: any) =>
  apiRequest('delete', url, request);

// function to execute the http post request
const post = (url: string, request?: any) => apiRequest('post', url, request);

// function to execute the http put request
const put = (url: string, request?: any) => apiRequest('put', url, request);

export default {
  get,
  delete: deleteRequest,
  post,
  put,
};
