import { z } from 'zod';

/**
 * Provider identifier for supported LLM providers
 * Must match Rust ProviderId enum
 */
export const ProviderIdSchema = z.enum(['ChatGPT', 'Gemini', 'Claude']);
export type ProviderId = z.infer<typeof ProviderIdSchema>;

/**
 * Status of a prompt submission
 * Must match Rust SubmissionStatus enum
 */
export const SubmissionStatusSchema = z.enum([
  'Pending',
  'InProgress',
  'Retrying',
  'Success',
  'Failed'
]);
export type SubmissionStatus = z.infer<typeof SubmissionStatusSchema>;

/**
 * Error types that can occur during submission
 * Must match Rust SubmissionErrorType enum
 */
export const SubmissionErrorTypeSchema = z.enum([
  'Timeout',
  'NetworkError',
  'AuthenticationError',
  'RateLimitError',
  'ElementNotFound',
  'InjectionFailed'
]);
export type SubmissionErrorType = z.infer<typeof SubmissionErrorTypeSchema>;

/**
 * Standard error returned from Tauri commands
 * Must match Rust CommandError struct
 */
export const CommandErrorSchema = z.object({
  code: z.string(),
  message: z.string()
});
export type CommandError = z.infer<typeof CommandErrorSchema>;

/**
 * Provider information
 */
export const ProviderSchema = z.object({
  id: ProviderIdSchema,
  name: z.string(),
  url: z.string(),
  is_selected: z.boolean(),
  is_authenticated: z.boolean(),
  selector_config_id: z.string()
});
export type Provider = z.infer<typeof ProviderSchema>;

/**
 * Layout type for provider panel arrangement
 */
export const LayoutTypeSchema = z.enum(['Full', 'VerticalSplit', 'Grid']);
export type LayoutType = z.infer<typeof LayoutTypeSchema>;

/**
 * Panel dimensions (percentages from 0.0 to 1.0)
 */
export const PanelDimensionSchema = z.object({
  provider_id: ProviderIdSchema,
  x: z.number().min(0).max(1),
  y: z.number().min(0).max(1),
  width: z.number().min(0).max(1),
  height: z.number().min(0).max(1)
});
export type PanelDimension = z.infer<typeof PanelDimensionSchema>;

/**
 * Layout configuration
 */
export const LayoutConfigurationSchema = z.object({
  provider_count: z.number().min(1).max(3),
  layout_type: LayoutTypeSchema,
  panel_dimensions: z.array(PanelDimensionSchema)
});
export type LayoutConfiguration = z.infer<typeof LayoutConfigurationSchema>;

/**
 * Submission state
 */
export const SubmissionSchema = z.object({
  id: z.string(), // UUID
  provider_id: ProviderIdSchema,
  prompt_content: z.string(),
  status: SubmissionStatusSchema,
  attempt_count: z.number().min(0).max(2),
  error_type: SubmissionErrorTypeSchema.optional(),
  error_message: z.string().optional(),
  started_at: z.string().optional(), // ISO 8601
  completed_at: z.string().optional() // ISO 8601
});
export type Submission = z.infer<typeof SubmissionSchema>;
