import http from "k6/http";
import exec from 'k6/execution';
import { check, sleep } from "k6";

const BASE_URL = 'https://tick.jakobfrenzel.com/todos';

export const options = {
  scenarios: {
    constant_rps: {
      executor: 'constant-arrival-rate',
      rate: 1000,
      timeUnit: '1s',
      duration: '1m',
      preAllocatedVUs: 100,
      maxVUs: 100,
    },
  },
};

export function setup() {
  let res = http.get(BASE_URL);
  if (res.status !== 200) {
    exec.test.abort(`Got unexpected status code ${res.status} when trying to setup. Exiting.`);
  }
}

export default function() {
  let res = http.get(`${BASE_URL}?count=1`);
  check(res, {
    'is status 200': (r) => r.status === 200,
  });
}
