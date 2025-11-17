// TypeScript type definitions mirroring Rust types
// These types ensure type safety across the Tauri IPC boundary

import { z } from 'zod';

/**
 * Provider identifier enum for the three supported LLM providers
 */
export enum ProviderId {
  ChatGPT = 'ChatGPT',
  Gemini = 'Gemini',
  Claude = 'Claude',
}

export const ProviderIdSchema = z.nativeEnum(ProviderId);

/**
 * Status of a prompt submission to a provider
 */
export enum SubmissionStatus {
  Pending = 'Pending',
  InProgress = 'InProgress',
  Retrying = 'Retrying',
  Success = 'Success',
  Failed = 'Failed',
}

export const SubmissionStatusSchema = z.nativeEnum(SubmissionStatus);

/**
 * Error types that can occur during prompt submission
 */
export enum SubmissionErrorType {
  Timeout = 'Timeout',
  NetworkError = 'NetworkError',
  AuthenticationError = 'AuthenticationError',
  RateLimitError = 'RateLimitError',
  ElementNotFound = 'ElementNotFound',
  InjectionFailed = 'InjectionFailed',
}

export const SubmissionErrorTypeSchema = z.nativeEnum(SubmissionErrorType);

/**
 * Standard error type for Tauri commands
 */
export interface CommandError {
  code: string;
  message: string;
}

export const CommandErrorSchema = z.object({
  code: z.string(),
  message: z.string(),
});

/**
 * Represents an LLM provider with its configuration and state
 */
export interface Provider {
  id: ProviderId;
  name: string;
  url: string;
  is_selected: boolean;
  is_authenticated: boolean;
  selector_config_id: string;
}

export const ProviderSchema = z.object({
  id: ProviderIdSchema,
  name: z.string(),
  url: z.string(),
  is_selected: z.boolean(),
  is_authenticated: z.boolean(),
  selector_config_id: z.string(),
});

/**
 * Layout type based on number of selected providers
 */
export enum LayoutType {
  Full = 'Full',
  VerticalSplit = 'VerticalSplit',
  Grid = 'Grid',
}

export const LayoutTypeSchema = z.nativeEnum(LayoutType);

/**
 * Panel dimensions for split-screen layout
 */
export interface PanelDimension {
  provider_id: ProviderId;
  x: number;
  y: number;
  width: number;
  height: number;
}

export const PanelDimensionSchema = z.object({
  provider_id: ProviderIdSchema,
  x: z.number().min(0).max(1),
  y: z.number().min(0).max(1),
  width: z.number().min(0).max(1),
  height: z.number().min(0).max(1),
});

/**
 * Layout configuration for provider panels
 */
export interface LayoutConfiguration {
  provider_count: number;
  layout_type: LayoutType;
  panel_dimensions: PanelDimension[];
}

export const LayoutConfigurationSchema = z.object({
  provider_count: z.number().min(1).max(3),
  layout_type: LayoutTypeSchema,
  panel_dimensions: z.array(PanelDimensionSchema),
});

/**
 * Submission entity tracking prompt submission to a provider
 */
export interface Submission {
  id: string;
  provider_id: ProviderId;
  prompt_content: string;
  status: SubmissionStatus;
  attempt_count: number;
  error_type?: SubmissionErrorType;
  error_message?: string;
  started_at?: string;
  completed_at?: string;
}

export const SubmissionSchema = z.object({
  id: z.string(),
  provider_id: ProviderIdSchema,
  prompt_content: z.string(),
  status: SubmissionStatusSchema,
  attempt_count: z.number().min(0).max(2),
  error_type: SubmissionErrorTypeSchema.optional(),
  error_message: z.string().optional(),
  started_at: z.string().optional(),
  completed_at: z.string().optional(),
});
