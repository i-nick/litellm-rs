#!/usr/bin/env python3
"""
Model Sync Script for litellm-rs

This script fetches the latest model information from OpenRouter API
and generates a report of models that need to be updated in the codebase.

Usage:
    python scripts/sync_models.py [--provider PROVIDER] [--output FORMAT]

Options:
    --provider      Filter by provider (openai, anthropic, google, deepseek, etc.)
    --output        Output format: report (default), json, rust, update
    --data-file     Path to model_prices.json (default: data/model_prices.json)

Examples:
    # Generate markdown report
    python scripts/sync_models.py

    # Get OpenAI models as JSON
    python scripts/sync_models.py --provider openai --output json

    # Update data/model_prices.json directly
    python scripts/sync_models.py --output update
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from datetime import datetime
from typing import Any, Dict, List, Optional
from urllib.request import urlopen
from urllib.error import URLError

OPENROUTER_API = "https://openrouter.ai/api/v1/models"
DEFAULT_DATA_FILE = "data/model_prices.json"

# Provider mapping from OpenRouter to our codebase
PROVIDER_MAPPING = {
    "openai": "openai",
    "anthropic": "anthropic",
    "google": "gemini",
    "deepseek": "deepseek",
    "meta-llama": "meta_llama",
    "mistralai": "mistral",
    "cohere": "cohere",
    "x-ai": "xai",
    "amazon": "amazon_nova",
    "qwen": "qwen",
    "moonshotai": "moonshot",
}

# Models we track (add more as needed)
TRACKED_PROVIDERS = [
    "openai", "anthropic", "google", "deepseek",
    "meta-llama", "mistralai", "cohere", "x-ai",
    "qwen", "moonshotai", "amazon"
]


def fetch_openrouter_models() -> List[Dict[str, Any]]:
    """Fetch all models from OpenRouter API."""
    try:
        with urlopen(OPENROUTER_API, timeout=30) as response:
            data = json.loads(response.read().decode())
            return data.get("data", [])
    except URLError as e:
        print(f"Error fetching models: {e}", file=sys.stderr)
        sys.exit(1)


def parse_model_info(model: Dict[str, Any]) -> Dict[str, Any]:
    """Parse model information from OpenRouter format."""
    model_id = model.get("id", "")
    parts = model_id.split("/")
    provider = parts[0] if len(parts) > 1 else "unknown"
    name = parts[1] if len(parts) > 1 else model_id

    pricing = model.get("pricing", {})
    architecture = model.get("architecture", {})

    return {
        "id": model_id,
        "provider": provider,
        "name": name,
        "display_name": model.get("name", name),
        "context_length": model.get("context_length", 0),
        "input_cost_per_million": float(pricing.get("prompt", 0)) * 1_000_000,
        "output_cost_per_million": float(pricing.get("completion", 0)) * 1_000_000,
        "supports_tools": "tools" in model.get("supported_parameters", []),
        "supports_vision": "image" in architecture.get("modality", ""),
        "supports_audio": "audio" in architecture.get("modality", ""),
        "created": model.get("created"),
    }


def group_by_provider(models: List[Dict[str, Any]]) -> Dict[str, List[Dict[str, Any]]]:
    """Group models by provider."""
    grouped: Dict[str, List[Dict[str, Any]]] = {}
    for model in models:
        provider = model["provider"]
        if provider not in grouped:
            grouped[provider] = []
        grouped[provider].append(model)
    return grouped


def generate_report(models: List[Dict[str, Any]], provider_filter: Optional[str] = None) -> str:
    """Generate a markdown report of models."""
    grouped = group_by_provider(models)

    lines = [
        "# OpenRouter Model Report",
        f"Generated: {datetime.now().isoformat()}",
        "",
    ]

    for provider in sorted(grouped.keys()):
        if provider_filter and provider != provider_filter:
            continue
        if provider not in TRACKED_PROVIDERS:
            continue

        provider_models = grouped[provider]
        lines.append(f"## {provider.upper()}")
        lines.append("")
        lines.append("| Model | Context | Input $/M | Output $/M | Tools | Vision |")
        lines.append("|-------|---------|-----------|------------|-------|--------|")

        for model in sorted(provider_models, key=lambda x: x["name"]):
            tools = "✓" if model["supports_tools"] else ""
            vision = "✓" if model["supports_vision"] else ""
            lines.append(
                f"| {model['name']} | {model['context_length']:,} | "
                f"${model['input_cost_per_million']:.2f} | "
                f"${model['output_cost_per_million']:.2f} | "
                f"{tools} | {vision} |"
            )
        lines.append("")

    return "\n".join(lines)


def generate_rust_snippet(model: Dict[str, Any]) -> str:
    """Generate Rust code snippet for a model."""
    return f'''
            // {model['display_name']}
            (
                "{model['name']}",
                "{model['display_name']}",
                {model['context_length']},
                Some(16384),
                {model['input_cost_per_million'] / 1_000_000:.8f}, // ${model['input_cost_per_million']:.2f}/1M input
                {model['output_cost_per_million'] / 1_000_000:.8f}, // ${model['output_cost_per_million']:.2f}/1M output
            ),'''


def convert_to_litellm_format(model: Dict[str, Any]) -> Dict[str, Any]:
    """Convert model to LiteLLM-compatible JSON format."""
    provider = PROVIDER_MAPPING.get(model["provider"], model["provider"])

    result = {
        "input_cost_per_token": model["input_cost_per_million"] / 1_000_000,
        "output_cost_per_token": model["output_cost_per_million"] / 1_000_000,
        "max_input_tokens": model["context_length"],
        "max_output_tokens": 16384,  # Default
        "litellm_provider": provider,
        "mode": "chat",
    }

    if model["supports_tools"]:
        result["supports_function_calling"] = True
    if model["supports_vision"]:
        result["supports_vision"] = True
    if model["supports_audio"]:
        result["supports_audio_input"] = True
        result["supports_audio_output"] = True

    return result


def update_data_file(models: List[Dict[str, Any]], data_file: str) -> int:
    """Update the model_prices.json file with new data."""
    # Load existing data
    try:
        with open(data_file) as f:
            existing = json.load(f)
    except FileNotFoundError:
        existing = {"_metadata": {}}

    # Update metadata
    existing["_metadata"] = {
        "version": "1.0.0",
        "updated_at": datetime.utcnow().isoformat() + "Z",
        "source": OPENROUTER_API
    }

    # Track changes
    added = 0
    updated = 0

    for model in models:
        if model["provider"] not in TRACKED_PROVIDERS:
            continue

        key = model["id"]
        new_data = convert_to_litellm_format(model)

        if key in existing:
            # Check if data changed
            if existing[key] != new_data:
                existing[key] = new_data
                updated += 1
        else:
            existing[key] = new_data
            added += 1

    # Write back
    with open(data_file, "w") as f:
        json.dump(existing, f, indent=2, sort_keys=True)

    print(f"Updated {data_file}: {added} added, {updated} updated", file=sys.stderr)
    return added + updated


def main():
    parser = argparse.ArgumentParser(description="Sync models from OpenRouter")
    parser.add_argument("--provider", help="Filter by provider")
    parser.add_argument("--output", choices=["report", "json", "rust", "update"], default="report")
    parser.add_argument("--data-file", default=DEFAULT_DATA_FILE, help="Path to model_prices.json")
    args = parser.parse_args()

    print("Fetching models from OpenRouter...", file=sys.stderr)
    raw_models = fetch_openrouter_models()
    models = [parse_model_info(m) for m in raw_models]

    print(f"Found {len(models)} models", file=sys.stderr)

    if args.output == "json":
        if args.provider:
            models = [m for m in models if m["provider"] == args.provider]
        print(json.dumps(models, indent=2))
    elif args.output == "rust":
        if args.provider:
            models = [m for m in models if m["provider"] == args.provider]
        for model in models:
            print(generate_rust_snippet(model))
    elif args.output == "update":
        if args.provider:
            models = [m for m in models if m["provider"] == args.provider]
        changes = update_data_file(models, args.data_file)
        print(f"Total changes: {changes}")
    else:
        print(generate_report(models, args.provider))


if __name__ == "__main__":
    main()
