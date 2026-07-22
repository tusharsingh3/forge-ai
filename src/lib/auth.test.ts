import { describe, expect, it } from 'vitest';
import { sessionNeedsRefresh } from './auth';

describe('session refresh window',()=>{
  it('keeps a session with more than thirty seconds remaining',()=>expect(sessionNeedsRefresh(1_031_000,1_000_000)).toBe(false));
  it('refreshes at the thirty second boundary',()=>expect(sessionNeedsRefresh(1_030_000,1_000_000)).toBe(true));
  it('refreshes expired sessions',()=>expect(sessionNeedsRefresh(999_000,1_000_000)).toBe(true));
});
