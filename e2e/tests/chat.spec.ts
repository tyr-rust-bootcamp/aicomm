import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';
import { AnalyticsEvent, AnalyticsEventSchema } from '../gen/messages_pb';
import { fromBinary } from "@bufbuild/protobuf";

const URL_TO_CAPTURE = ['/api/event'];

function decodeEvent(data: Uint8Array) {
  const event = fromBinary(AnalyticsEventSchema, data);
  return event;
}

test('test', async ({ page }) => {
  let requests: { url: string; data: AnalyticsEvent }[] = [];

  page.on('request', (req) => {
    if (URL_TO_CAPTURE.some((url) => req.url().includes(url))) {
      let url = req.url();
      let data = decodeEvent(req.postDataBuffer());
      console.log('>>', req.method(), url, data);
      requests.push({ url, data });
    }
  });

  page.on('response', (res) => {

    if (URL_TO_CAPTURE.some((url) => res.url().includes(url))) {
      console.log('<<', res.status(), res.url());
    }
  });

  // Recording...
  await page.goto('http://localhost:1420/login');

  // find input[id="email"] and fill it with 'tchen@acme.org'
  await page.locator('input[id="email"]').fill('tchen@acme.org');

  // find input[id="password"] and fill it with '123456'
  await page.locator('input[id="password"]').fill('123456');

  // find button[type="submit"] and click it
  await page.locator('button[type="submit"]').click();

  // expect the page to have a textarea to input message
  let textInput = page.locator('textarea[placeholder="Type a message..."]');
  await expect(textInput).toBeVisible();
  await expect(textInput).toBeEmpty();

  await page.getByText('Alice Chen').click();
  const id = uuidv4();
  const msg = `Hello, This is my secret: ${id}`;
  await textInput.fill(msg);
  await page.getByRole('button').nth(3).click();

  // expect "Alice, I want to share you a good news!" to be visible
  await expect(page.getByText(msg)).toBeVisible();

  // take a screenshot
  await page.screenshot({ path: `./screenshots/${id}.png` });

  expect(requests.length).toEqual(4);
  expect(requests[0].data.eventType.case).toEqual('appStart');
  expect(requests[1].data.eventType.case).toEqual('userLogin');
  expect(requests[2].data.eventType.case).toEqual('navigation');
  expect(requests[3].data.eventType.case).toEqual('messageSent');
});
