import unittest
from pathlib import Path

class WorkflowTextTests(unittest.TestCase):
    def test_shell_operators_are_not_html_entities(self):
        workflow = Path(__file__).resolve().parents[2] / '.github/workflows/draft-release-build-candidate.yml'
        text = workflow.read_text()
        for entity in ('&gt;', '&lt;', '&amp;'):
            self.assertNotIn(entity, text)
        self.assertIn('sha256sum pong-typescript-browser.zip > SHA256SUMS', text)

if __name__ == '__main__': unittest.main()
