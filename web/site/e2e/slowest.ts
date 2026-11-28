// Prints the ten slowest tests of the run, each with its time, after the list reporter.

import type {FullResult, Reporter, TestCase, TestResult} from '@playwright/test/reporter';

export default class Slowest implements Reporter {
  private times: {title: string; ms: number}[] = [];

  onTestEnd(test: TestCase, result: TestResult): void {
    this.times.push({title: `${test.parent.project()?.name ?? ''} › ${test.title}`, ms: result.duration});
  }

  onEnd(_result: FullResult): void {
    const top = this.times.sort((a, b) => b.ms - a.ms).slice(0, 10);
    if (!top.length) return;
    console.log('\nThe ten slowest tests:');
    for (const t of top) console.log(`  ${(t.ms / 1000).toFixed(1).padStart(5)} s  ${t.title}`);
  }

  printsToStdio(): boolean {
    return false;
  }
}
