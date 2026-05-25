import { describe, expect, it, vi } from 'vitest';
import { tauri } from '../../src/services/tauri';

const invoke = vi.fn();

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invoke(...args),
}));

describe('TauriService', () => {
  it('polls submission status with the camelCase argument expected by Tauri', async () => {
    invoke.mockResolvedValue({
      id: 'submission-1',
      provider_id: 'ChatGPT',
      prompt_content: 'test',
      status: 'Success',
      attempt_count: 1,
    });

    await tauri.getSubmissionStatus('submission-1');

    expect(invoke).toHaveBeenCalledWith('get_submission_status', {
      submissionId: 'submission-1',
    });
  });
});
